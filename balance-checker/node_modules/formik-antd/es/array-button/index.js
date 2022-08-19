import { __rest } from "tslib";
import { FieldArray } from 'formik';
import * as React from 'react';
import Button from 'antd/lib/button';
export function ArrayButton(_a) {
    var { name, onClick } = _a, restProps = __rest(_a, ["name", "onClick"]);
    return (React.createElement(FieldArray, { name: name }, (array) => React.createElement(Button, Object.assign({}, restProps, { onClick: () => onClick(array) }))));
}
export default ArrayButton;
//# sourceMappingURL=index.js.map