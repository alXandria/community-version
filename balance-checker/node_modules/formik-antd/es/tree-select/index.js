import { __rest } from "tslib";
import { TreeSelect as $TreeSelect } from 'antd';
import * as React from 'react';
import Field from '../field';
export const TreeSelect = (_a) => {
    var { name, validate, fast, onChange, onBlur } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange", "onBlur"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value }, form }) => (React.createElement($TreeSelect, Object.assign({ value: value, onBlur: (event) => {
            form.setFieldTouched(name);
            onBlur && onBlur(event);
        }, onChange: (value, node, extra) => {
            form.setFieldValue(name, value);
            onChange && onChange(value, node, extra);
        } }, restProps)))));
};
export default TreeSelect;
TreeSelect.TreeNode = $TreeSelect.TreeNode;
//# sourceMappingURL=index.js.map