import { __rest } from "tslib";
import { AutoComplete as $AutoComplete } from 'antd';
import * as React from 'react';
import Field from '../field';
export const AutoComplete = (_a) => {
    var { name, validate, fast, onChange, onBlur } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange", "onBlur"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value }, form }) => (React.createElement($AutoComplete, Object.assign({ value: value, onChange: (value, option) => {
            form.setFieldValue(name, value != null ? value.valueOf() : value);
            onChange && onChange(value, option);
        }, onBlur: (value) => {
            form.setFieldTouched(name);
            onBlur && onBlur(value);
        } }, restProps)))));
};
export default AutoComplete;
//# sourceMappingURL=index.js.map