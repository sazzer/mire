import React from "react";
import { User } from "../api/users";
import { useTranslation } from "react-i18next";

/**
 * Props for the list of authentications
 */
export interface AuthenticationsProps {
  user: User;
}

export const Authentications: React.FC<AuthenticationsProps> = ({ user }) => {
  const { t } = useTranslation();

  const listEntries = user.authentications.map((authentication) => {
    return (
      <li
        className="list-group-item d-flex justify-content-between align-items-center"
        key={`${authentication.authenticationProvider}-${authentication.authenticationId}`}
      >
        {authentication.displayName}
        <span className={`btn-${authentication.authenticationProvider} badge`}>
          <span
            className={`fa fa-${authentication.authenticationProvider}`}
          ></span>{" "}
          {t(
            `profile.authentications.${authentication.authenticationProvider}`
          )}
        </span>
      </li>
    );
  });

  return <ul className="list-group">{listEntries}</ul>;
};
