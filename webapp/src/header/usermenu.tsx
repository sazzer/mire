import React from "react";
import { useTranslation } from "react-i18next";

/**
 * The User Menu dropdown appearing in the header bar.
 */
export const UserMenu = () => {
  return (
    <li className="nav-item dropdown">
      <a
        className="nav-link dropdown-toggle"
        href="#"
        id="userMenuDropdown"
        role="button"
        data-toggle="dropdown"
        aria-haspopup="true"
        aria-expanded="false"
      >
        Graham
      </a>
      <div
        className="dropdown-menu dropdown-menu-right"
        aria-labelledby="userMenuDropdown"
      >
        <a className="dropdown-item" href="#">
          User Profile
        </a>
        <div className="dropdown-divider"></div>
        <a className="dropdown-item" href="#">
          Log Out
        </a>
      </div>
    </li>
  );
};
