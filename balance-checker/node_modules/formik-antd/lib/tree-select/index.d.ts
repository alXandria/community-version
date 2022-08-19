import * as React from 'react';
import { FormikFieldProps } from '../FieldProps';
import { TreeSelectProps as $TreeSelectProps } from 'antd/lib/tree-select';
export declare type TreeSelectProps = FormikFieldProps & $TreeSelectProps<any> & {
    children?: React.ReactNode;
};
export declare const TreeSelect: {
    ({ name, validate, fast, onChange, onBlur, ...restProps }: TreeSelectProps): JSX.Element;
    TreeNode: React.FC<import("rc-tree-select/lib/TreeNode").TreeNodeProps>;
};
export default TreeSelect;
//# sourceMappingURL=index.d.ts.map