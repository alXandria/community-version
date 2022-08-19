import {Formik} from "formik";
import {Form, FormItem} from "formik-antd";
import React from "react";
import Search from "../../../forms/Search";
import {searchValidationSchema} from "../../../forms/validationSchemas";

interface
FormCheckBalanceProps
{
  readonly
  setContractAddress: (value: React.SetStateAction<string>) => void;
}

export function FormCheckBalance({setContractAddress}: FormCheckBalanceProps): JSX.Element {
  return (
    <Formik
      initialValues={{address: ""}}
      validationSchema={searchValidationSchema}
      onSubmit={(values) => {
        setContractAddress(values.address);
      }}
    >
      {(formikProps) => (
        <Form>
          <FormItem name="address">
            <Search
              name="address"
              placeholder="Enter contract address"
              enterButton
              onSearch={formikProps.submitForm}
            />
          </FormItem>
        </Form>
      )}
    </Formik>
  );
}