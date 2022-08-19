import * as React from 'react';
import { FieldConfig } from 'formik';
import { FormItemProps as $FormItemProps } from 'antd/lib/form/FormItem';
export declare type FormItemProps = {
    showValidateSuccess?: boolean;
    showInitialErrorAfterTouched?: boolean;
    children: React.ReactNode;
} & {
    name: string;
} & $FormItemProps & Pick<FieldConfig, 'validate'>;
export declare const FormItem: ({ name, showValidateSuccess, showInitialErrorAfterTouched, children, validate, ...restProps }: FormItemProps) => JSX.Element;
export default FormItem;
//# sourceMappingURL=index.d.ts.map