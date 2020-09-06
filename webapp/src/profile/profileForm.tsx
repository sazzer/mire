import React, { useState } from "react";
import { User, saveUser } from "../api/users";

import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";

/**
 * Props for the actual form
 */
export interface ProfileFormProps {
  user: User;
}

interface ProfileFormFields {
  email: string;
  displayName: string;
  created: string;
  updated: string;
}

/**
 * Component to render the actual form to view the user profile
 * @param user The user to work with
 */
export const ProfileForm: React.FC<ProfileFormProps> = ({ user }) => {
  const { t } = useTranslation();
  const { register, handleSubmit } = useForm<ProfileFormFields>({
    defaultValues: {
      email: user.email,
      displayName: user.displayName,
      created: t("common.formattedDateTime", { date: user.created }),
      updated: t("common.formattedDateTime", { date: user.updated }),
    },
  });
  const [state, setState] = useState<"INITIAL" | "SAVING" | "SAVED" | "ERROR">(
    "INITIAL"
  );

  const onSubmit = (data: ProfileFormFields) => {
    setState("SAVING");
    const newUser = {
      ...user,
      email: data.email,
      displayName: data.displayName,
    };

    saveUser(newUser)
      .then(() => {
        setState("SAVED");
      })
      .catch(() => {
        setState("ERROR");
      });
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <fieldset disabled={state === "SAVING"}>
        <div className="form-group">
          <label htmlFor="userProfileDisplayName">
            {t("profile.profile.displayName.label")}
          </label>
          <input
            type="text"
            className="form-control"
            id="userProfileDisplayName"
            name="displayName"
            autoFocus
            required
            ref={register({ required: true })}
            aria-describedby="userProfileDisplayNameHelp"
          />
          <small
            id="userProfileDisplayNameHelp"
            className="form-text text-muted"
          >
            {t("profile.profile.displayName.help")}
          </small>
        </div>

        <div className="form-group">
          <label htmlFor="userProfileEmail">
            {t("profile.profile.email.label")}
          </label>
          <input
            type="email"
            className="form-control"
            id="userProfileEmail"
            name="email"
            required
            ref={register({ required: true })}
            aria-describedby="userProfileEmailHelp"
          />
          <small id="userProfileEmailHelp" className="form-text text-muted">
            {t("profile.profile.email.help")}
          </small>
        </div>

        <div className="form-row">
          <div className="col-md-6 mb-3">
            <div className="form-group">
              <label htmlFor="userProfileCreated">
                {t("profile.profile.created.label")}
              </label>
              <input
                type="created"
                className="form-control-plaintext"
                id="userProfileCreated"
                name="created"
                ref={register}
                aria-describedby="userProfileCreatedHelp"
                readOnly
              />
            </div>
          </div>
          <div className="col-md-6 mb-3">
            <div className="form-group">
              <label htmlFor="userProfileUpdated">
                {t("profile.profile.updated.label")}
              </label>
              <input
                type="updated"
                className="form-control-plaintext"
                id="userProfileUpdated"
                name="updated"
                ref={register}
                aria-describedby="userProfileUpdatedHelp"
                readOnly
              />
            </div>
          </div>
        </div>

        <div className="form-group">
          <button type="submit" className="btn btn-primary">
            {t("profile.profile.actions.submit")}
          </button>
        </div>
      </fieldset>

      {state === "SAVING" && (
        <div className="alert alert-primary" role="alert">
          {t("profile.profile.alert.saving")}
        </div>
      )}
      {state === "SAVED" && (
        <div className="alert alert-success" role="alert">
          {t("profile.profile.alert.saved")}
        </div>
      )}
      {state === "ERROR" && (
        <div className="alert alert-danger" role="alert">
          {t("profile.profile.alert.error")}
        </div>
      )}
    </form>
  );
};
