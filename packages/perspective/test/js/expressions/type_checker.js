/******************************************************************************
 *
 * Copyright (c) 2017, the Perspective Authors.
 *
 * This file is part of the Perspective library, distributed under the terms of
 * the Apache License 2.0.  The full license can be found in the LICENSE file.
 *
 */

const expressions_common = require("./common.js");

/**
 * Tests the functionality of `View`-based expressions, specifically that
 * existing column/view semantics (pivots, aggregates, columns, sorts, filters)
 * continue to be functional on expressions.
 */
module.exports = (perspective) => {
    describe("Type checking", () => {
        it("simple expressions", async () => {
            const table = await perspective.table(expressions_common.data);
            const validate = await table.type_check_expressions([
                // '"x"',
                // '"y"',
                // '"z"',
                // "1 * 100",
                // "// xyz\nsqrt(100)",
                "var x := 1; x * 'abc'",
                // "-1921.213 * 'abc'",
                // "'abc' * 1",
                // "'abc' * 256.12345",
                // '"x" * 1',
                // '"x" + "y"',
                // "\"x\" * 'abcdef'",
                // 'abs("z")',
            ]);

            const schema = validate.expression_schema;
            const errors = validate.errors;

            console.error(validate);

            expect(schema).toEqual({
                '"x"': "integer",
                '"y"': "string",
                '"z"': "boolean",
                "1 * 100": "float",
                xyz: "float",
            });

            await table.delete();
        });
    });
};
