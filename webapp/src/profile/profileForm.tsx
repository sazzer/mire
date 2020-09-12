import { FormProvider, useForm } from "react-hook-form";
import { Problem, ValidationError } from "../api/http";
import React, { useState } from "react";
import { User, saveUser } from "../api/users";

import { FormField } from "../components/form/input";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

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
  const currentUser = useUser();

  const { t } = useTranslation();
  const form = useForm<ProfileFormFields>({
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
    form.clearErrors();
    setState("SAVING");
    const newUser = {
      ...user,
      email: data.email,
      displayName: data.displayName,
    };

    saveUser(newUser)
      .then((savedUser) => {
        setState("SAVED");
        if (currentUser.hasUser) {
          return currentUser.reload();
        } else {
          return savedUser;
        }
      })
      .then((savedUser) => {
        form.setValue("email", savedUser.email);
        form.setValue("displayName", savedUser.displayName);
        form.setValue(
          "updated",
          t("common.formattedDateTime", { date: savedUser.updated })
        );
      })
      .catch((e) => {
        setState("ERROR");
        if (e instanceof ValidationError) {
          e.getFieldErrors("email")
            .map((e) => e.type)
            .forEach((type) => form.setError("email", { type }));
          e.getFieldErrors("displayName")
            .map((e) => e.type)
            .forEach((type) => form.setError("displayName", { type }));
        } else if (e instanceof Problem) {
          if (e.type === "tag:mire/2020:users/problems/duplicate_email") {
            form.setError("email", { type: e.type });
          }
        }
      });
  };

  return (
    <FormProvider {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
        <fieldset disabled={state === "SAVING"}>
          <FormField
            id="userProfileDisplayName"
            name="displayName"
            autoFocus
            required
            label={t("profile.profile.displayName.label")}
            helpText={t("profile.profile.displayName.help")}
            errorPrefix="profile.profile.displayName.errors."
          />
          <FormField
            id="userProfileEmail"
            name="email"
            type="email"
            required
            label={t("profile.profile.email.label")}
            helpText={t("profile.profile.email.help")}
            errorPrefix="profile.profile.email.errors."
          />

          <div className="form-row">
            <div className="col-md-6 mb-3">
              <FormField
                id="userProfileCreated"
                name="created"
                label={t("profile.profile.created.label")}
                readOnly
              />
            </div>
            <div className="col-md-6 mb-3">
              <FormField
                id="userProfileUpdated"
                name="updated"
                label={t("profile.profile.updated.label")}
                readOnly
              />
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
    </FormProvider>
  );
};
