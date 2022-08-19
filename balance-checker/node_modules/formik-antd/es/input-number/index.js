import { __rest } from "tslib";
import { InputNumber as $InputNumber } from 'antd';
import * as React from 'react';
import Field from '../field';
export const InputNumber = (_a) => {
    var { name, validate, fast, onChange: $onChange, onBlur: $onBlur } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange", "onBlur"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value, onBlur }, form: { setFieldValue } }) => (React.createElement($InputNumber, Object.assign({ name: name, value: value, onChange: (value) => {
            setFieldValue(name, value);
            $onChange && $onChange(value);
        }, onBlur: (event) => {
            onBlur(event);
            $onBlur && $onBlur(event);
        } }, restProps)))));
};
export default InputNumber;
//# sourceMappingURL=index.js.map