import { Link, useHistory } from "react-router-dom";

import React from "react";
import { clearToken } from "../api/http/token";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

/**
 * The User Menu dropdown appearing in the header bar.
 */
export const UserMenu: React.FC = () => {
  const { t } = useTranslation();
  const { clearUserId } = useUser();
  const history = useHistory();

  const user = {
    name: "Graham",
  };

  const onLogOut = () => {
    clearUserId();
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
        aria-label={t("header.userMenu.label.full", user)}
      >
        {t("header.userMenu.label.simple", user)}
      </a>
      <div
        className="dropdown-menu dropdown-menu-right"
        aria-labelledby="userMenuDropdown"
      >
        <Link to="/profile" className="dropdown-item" role="menuitem">
          {t("header.userMenu.profile")}
        </Link>
        <div className="dropdown-divider"></div>
        <button className="dropdown-item" role="menuitem" onClick={onLogOut}>
          {t("header.userMenu.logout")}
        </button>
      </div>
    </li>
  );
};
