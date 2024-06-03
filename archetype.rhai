let context = #{};

context += prompt("Project Name:", "project-name", #{
    cases: [
        CasedIdentityCasedValue(PROGRAMMING_CASES),
        FixedKeyCasedValue("project-title", KebabCase),
    ],
});

context += prompt("Port:", "port", #{
    defaults_with: 8080,
    type: Int,
    min: 1024,
    max: 65534,
});

if switch_enabled("debug-context") {
    display(as_yaml(context));
}

render(Directory("contents/base"), context);

// return context
context