// Adopted from https://stackoverflow.com/a/12459801.
$(function () {
    "use strict";
    function headerID(title) {
        return "#" + title.toLowerCase().replace(/\s+/g, "-");
    }

    function onlyText(e) {
        return e.clone().children().remove().end().text();
    }

    function addSectionContents(section, list) {
		if (section.children("blockquote").length > 0) {
			section = section.find("blockquote > div").first();
		}

        var classes = [
            "class",
            "classmethod",
            "attribute",
            "method",
            "property",
            "function",
        ];

        classes.forEach(function (cls) {
            var dl = section.children("dl." + cls);
            var children = dl.children("dt");
            if (children.length === 0) {
                return;
            }

            var ul = $("<ul>");
            children.each(function (_, child) {
                var n = $(child).children('.descname').clone();
                var a = $("<a>").attr("href",
                    $(child).children('.headerlink').attr("href")
                );

                var li = $("<li>").append(a.append(n));
                addSectionContents($(child).parent().children("dd"), li);
                ul.append(li);
            });

            list.append(ul);
        });
    }

    function generateContents() {
        var currentIndex = $('li.current > ul.simple');
        $(":header > a.headerlink").each(function (_, header) {
            if ($(header).parents(":header").prop("tagName") === "H1") {
                return;
            }

            var section = $(header).parents(".section").first();
            var title = onlyText($(header).parent());
            var li = $("<li>");
            li.append($("<a>").attr("href", headerID(title)).text(title));
            addSectionContents(section, li);
            currentIndex.append(li);
        });
    }

    generateContents();
});
