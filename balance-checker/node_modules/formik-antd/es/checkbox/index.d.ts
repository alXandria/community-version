/// <reference types="react" />
import { CheckboxProps as $CheckboxProps } from 'antd/lib/checkbox/Checkbox';
import { FormikFieldProps } from '../FieldProps';
import { CheckboxGroupProps as $CheckboxGroupProps } from 'antd/lib/checkbox/Group';
export declare type CheckboxProps = FormikFieldProps & $CheckboxProps;
export declare const Checkbox: {
    ({ name, validate, fast, onChange, ...restProps }: CheckboxProps): JSX.Element;
    Group({ name, validate, onChange, ...restProps }: CheckboxGroupProps): JSX.Element;
};
export default Checkbox;
export declare type CheckboxGroupProps = FormikFieldProps & $CheckboxGroupProps;
//# sourceMappingURL=index.d.ts.map