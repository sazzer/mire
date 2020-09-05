import React from "react";
import { User } from "../api/users";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";

export interface ProfileFormProps {
  user: User;
}

export const ProfileForm: React.FC<ProfileFormProps> = ({ user }) => {
  const { t } = useTranslation();
  const { register, handleSubmit } = useForm({
    defaultValues: {
      email: user.email,
      displayName: user.displayName,
      created: t("common.formattedDateTime", user.created),
      updated: t("common.formattedDateTime", user.updated),
    },
  });
  const onSubmit = (data: any) => console.log(data);

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <div className="form-group">
        <label htmlFor="userProfileDisplayName">
          {t("profile.profile.displayName.label")}
        </label>
        <input
          type="text"
          className="form-control"
          id="userProfileDisplayName"
          name="displayName"
          ref={register}
          aria-describedby="userProfileDisplayNameHelp"
        />
        <small id="userProfileDisplayNameHelp" className="form-text text-muted">
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
          ref={register}
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

      <button type="submit" className="btn btn-primary">
        {t("profile.profile.actions.submit")}
      </button>
    </form>
  );
};
