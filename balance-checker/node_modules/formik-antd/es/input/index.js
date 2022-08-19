import { __rest } from "tslib";
import { Input as $Input } from 'antd';
import * as React from 'react';
import Field from '../field';
const Input = React.forwardRef((_a, ref) => {
    var { name, validate, fast, onChange: $onChange, onBlur: $onBlur } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange", "onBlur"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value, onChange, onBlur } }) => (React.createElement($Input, Object.assign({ ref: ref, name: name, value: value, onChange: (event) => {
            onChange(event);
            $onChange && $onChange(event);
        }, onBlur: (event) => {
            onBlur(event);
            $onBlur && $onBlur(event);
        } }, restProps)))));
});
const TypedInput = Input;
TypedInput.Password = React.forwardRef((_a, ref) => {
    var { name, validate, fast, onChange: $onChange, onBlur: $onBlur } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange", "onBlur"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value, onChange, onBlur } }) => (React.createElement($Input.Password, Object.assign({ ref: ref, name: name, value: value, onChange: (event) => {
            onChange(event);
            $onChange && $onChange(event);
        }, onBlur: (event) => {
            onBlur(event);
            $onBlur && $onBlur(event);
        } }, restProps)))));
});
TypedInput.TextArea = React.forwardRef((_a, ref) => {
    var { name, validate, fast, onChange: $onChange, onBlur: $onBlur } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange", "onBlur"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value, onChange, onBlur } }) => (React.createElement($Input.TextArea, Object.assign({ ref: ref, name: name, value: value, onChange: (event) => {
            onChange(event);
            $onChange && $onChange(event);
        }, onBlur: (event) => {
            onBlur(event);
            $onBlur && $onBlur(event);
        } }, restProps)))));
});
export { TypedInput as Input };
export default TypedInput;
//# sourceMappingURL=index.js.map