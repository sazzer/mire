import { Link } from "react-router-dom";
import React from "react";

/**
 * The User Menu dropdown appearing in the header bar.
 */
export const UserMenu: React.FC = () => {
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
        aria-label="User Menu for Graham"
      >
        Graham
      </a>
      <div
        className="dropdown-menu dropdown-menu-right"
        aria-labelledby="userMenuDropdown"
      >
        <Link to="/profile" className="dropdown-item" role="menuitem">
          User Profile
        </Link>
        <div className="dropdown-divider"></div>
        <button className="dropdown-item" role="menuitem">
          Log Out
        </button>
      </div>
    </li>
  );
};
