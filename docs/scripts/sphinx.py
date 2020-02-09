#!/usr/bin/env python3
import autopy
import inspect
import re
import sys


class DocstringMarkdownParser(object):
    """
    This is a simple wrapper that lets us write human-readable docstrings as
    Markdown without sacrificing documentation quality.
    """
    def __init__(self, modules: list):
        self.modules = modules

    @property
    def members(self) -> dict:
        modules = [__builtins__] + [sys.modules[name] for name in self.modules]
        return _getmembers(modules)

    @property
    def functions(self) -> list:
        return [x for x, y in self.members.items() if inspect.isfunction(y)]

    @property
    def classes(self) -> list:
        return [x for x, y in self.members.items() if inspect.isclass(y)]

    @property
    def tokens(self) -> list:
        members = inspect.getmembers(__builtins__)
        return [x for x, y in members if not inspect.isfunction(y) and
                not inspect.isclass(y)]

    @property
    def methods(self) -> list:
        classes = (y for x, y in self.members.items() if inspect.isclass(y))
        return _getmembers(classes)

    @staticmethod
    def _mono_re(pattern: str):
        return re.compile(r'`(%s)`' % pattern, re.MULTILINE)

    @property
    def token_re(self) -> object:
        return self._mono_re(_re_keywords(self.tokens))

    @property
    def function_re(self) -> object:
        return self._mono_re(_re_keywords(self.functions) +
                             _re_keywords(x + "()" for x in self.functions))

    @property
    def class_re(self) -> object:
        return self._mono_re(_re_keywords(self.classes))

    @property
    def module_re(self) -> object:
        return self._mono_re(_re_keywords(self.modules))

    def sub_method(self, matchobj: object) -> str:
        name = matchobj.group(1)
        if name.endswith("()"):
            name = name[:-2]
        method = self.methods.get(name)
        if method is None:
            return matchobj.group(0)

        fmt = r':attr:`%s`' if isinstance(method, property) else r':meth:`%s`'
        return fmt % name

    def transform_docstring(self,
                            docstr: str,
                            name: str,
                            what: str,
                            obj: object) -> str:
        # Tokenize keywords.
        docstr = self.token_re.sub(r':token:`\1`', docstr)

        # Automatically insert attributes for local hyperlinking.
        docstr = self.function_re.sub(r':func:`\1`', docstr)
        docstr = self.class_re.sub(r':class:`\1`', docstr)
        docstr = self.module_re.sub(r':module:`\1`', docstr)
        if what in ("method", "class", "attribute"):
            docstr = self._mono_re(r'[\w()]+').sub(self.sub_method, docstr)

        # Treat `text` as monospace (like Markdown).
        docstr = re.sub(r'([^:]|^)`(.*?)`', r'\1``\2``', docstr,
                        flags=re.MULTILINE | re.DOTALL)

        # Automatically italicize "Exceptions:"
        docstr = re.sub(r'(\b)(Exceptions:)', r'\1`\2`', docstr)

        # Automatically format code blocks.
        docstr = re.sub("(\s*\n)+((\n?(    |\t)(\w.*))+)",
                        "\n::\n" + r'\2', docstr, re.MULTILINE)

        return docstr


class Sphinx(object):
    def __init__(self, app):
        self.app = app
        self.modules = ["autopy.%s" % x for x in autopy.__all__]
        self.nodoc = set()
        self.parser = DocstringMarkdownParser(self.modules)

    def autodoc_process_docstring(self, app: str, what: str, name: str,
                                  obj: object, options: dict, lines: list):
        if name not in self.nodoc:
            if what == 'module':
                self.nodoc.add(name)

            lines[:] = self.parser.transform_docstring(
                "\n".join(lines), name, what, obj
            ).split("\n")
        else:
            del lines[:]

    def autodoc_process_signature(self, app: str, what: str, name: str,
                                  obj: object, options: dict, signature: str,
                                  return_annotation: str):
        module = inspect.getmodule(obj)
        if signature and module:
            # Remove superfluous module name from type signature.
            signature = re.sub(r'%s\.(\w+)' % re.escape(module.__name__),
                               r'\1', signature)
        return signature, return_annotation

    # See http://sphinx-doc.org/templating.html#rellinks.
    def filternav(self, rellinks: list) -> list:
        # Remove useless navigation.
        rellinks = [x for x in rellinks if str(x[3]) != "modules"]
        rellinks.append(("index", "Index", "index", "index"))

        # Sort previous < index < next.
        order = {"previous": 2, "index": 1, "next": 0}
        rellinks = sorted(rellinks, key=lambda x: order.get(str(x[3]), -1))
        return rellinks

    def add_jinja_filters(self, app):
        app.builder.templates.environment.filters['filternav'] = self.filternav

    def setup(self):
        self.app.connect('autodoc-process-docstring',
                         self.autodoc_process_docstring)
        self.app.connect('autodoc-process-signature',
                         self.autodoc_process_signature)
        self.app.connect('builder-inited', self.add_jinja_filters)


def _re_keywords(words: list) -> str:
    return "|".join(map(re.escape, words))


def _getmembers(objects: list) -> dict:
    return dict(sum((inspect.getmembers(x) for x in objects), []))
