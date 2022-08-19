import { __rest } from "tslib";
import { Mentions as $Mentions } from 'antd';
import * as React from 'react';
import Field from '../field';
export const Mentions = (_a) => {
    var { name, validate, fast, onChange: $onChange, onBlur: $onBlur } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange", "onBlur"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value, onChange, onBlur }, form }) => (React.createElement($Mentions, Object.assign({ name: name, value: value, onChange: (event) => {
            form.setFieldValue(name, event);
            $onChange && $onChange(event);
        }, onBlur: (e) => {
            onBlur(name);
            $onBlur && $onBlur(e);
        } }, restProps)))));
};
Mentions.Option = $Mentions.Option;
export default Mentions;
//# sourceMappingURL=index.js.map