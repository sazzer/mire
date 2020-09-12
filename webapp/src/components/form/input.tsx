import React from "react";
import { useFormContext } from "react-hook-form";
import { useTranslation } from "react-i18next";

export interface FormFieldProps {
  id: string;
  label: string;
  helpText?: string;
  name: string;
  type?: string;
  required?: boolean;
  autoFocus?: boolean;
  readOnly?: boolean;
  errorPrefix?: string;
}

export const FormField: React.FC<FormFieldProps> = ({
  id,
  label,
  helpText,
  name,
  type,
  required,
  autoFocus,
  readOnly,
  errorPrefix,
}) => {
  const { t } = useTranslation();
  const { register, errors } = useFormContext();
  const error = errors[name];

  let describedBy = [];
  let classes = [readOnly ? "form-control-plaintext" : "form-control"];
  if (error) {
    classes.push("is-invalid");
  }

  let helpArea = <></>;
  if (helpText) {
    const helpId = id + "Help";
    helpArea = (
      <small id={helpId} className="form-text text-muted">
        {helpText}
      </small>
    );
    describedBy.push(helpId);
  }

  let feedbackArea = <></>;
  if (error && errorPrefix) {
    const feedbackId = id + "Feedback";
    feedbackArea = (
      <div id={feedbackId} className="invalid-feedback">
        {t(errorPrefix + error.type)}
      </div>
    );
    describedBy.push(feedbackId);
  }

  return (
    <div className="form-group">
      <label htmlFor={id}>{label}</label>
      <input
        type={type || "text"}
        className={classes.join(" ")}
        id={id}
        name={name}
        autoFocus={autoFocus}
        readOnly={readOnly}
        required={required}
        ref={register({ required })}
        aria-describedby={describedBy.join(",")}
      />
      {feedbackArea}
      {helpArea}
    </div>
  );
};
