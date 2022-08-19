import { __rest } from "tslib";
import { Field } from 'formik';
import * as React from 'react';
import { Table as $Table } from 'antd';
// import { TableProps, WithStore } from 'antd/lib/table';
// RecordType extends object = any>(props: TableProps<RecordType>
// declare function Table<RecordType extends object = any>(props: TableProps<RecordType>): JSX.Element;
export function Table(_a) {
    var { name } = _a, restProps = __rest(_a, ["name"]);
    return (React.createElement(Field, { name: name }, ({ field }) => {
        return React.createElement($Table, Object.assign({ dataSource: field.value || [] }, restProps));
    }));
}
export default Table;
//# sourceMappingURL=index.js.map