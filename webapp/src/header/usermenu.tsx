import { Link, useHistory } from "react-router-dom";

import React from "react";
import { User } from "../api/users";
import { clearToken } from "../api/http/token";
import { useTranslation } from "react-i18next";

/**
 * Props needed for the User Menu
 */
export interface UserMenuProps {
  user: User;
  onLogout: () => void;
}

/**
 * The User Menu dropdown appearing in the header bar.
 */
export const UserMenu: React.FC<UserMenuProps> = ({ user, onLogout }) => {
  const { t } = useTranslation();
  const history = useHistory();

  const doLogout = () => {
    onLogout();
    clearToken();
    history.push("/");
  };

  /* eslint-disable jsx-a11y/anchor-is-valid */
  return (
    <li className="nav-item dropdown" role="menu">
      <a
        href="#"
        className="nav-link dropdown-toggle"
        id="userMenuDropdown"
        role="button"
        data-toggle="dropdown"
        aria-haspopup="true"
        aria-expanded="false"
        aria-label={t("header.userMenu.label.full", {
          name: user?.displayName,
        })}
      >
        {t("header.userMenu.label.simple", { name: user?.displayName })}
      </a>
      <div
        className="dropdown-menu dropdown-menu-right"
        aria-labelledby="userMenuDropdown"
      >
        <Link to="/profile" className="dropdown-item" role="menuitem">
          {t("header.userMenu.profile")}
        </Link>
        <div className="dropdown-divider"></div>
        <button className="dropdown-item" role="menuitem" onClick={doLogout}>
          {t("header.userMenu.logout")}
        </button>
      </div>
    </li>
  );
};
