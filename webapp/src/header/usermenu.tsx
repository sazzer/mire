import { Link } from "react-router-dom";
import React from "react";

/**
 * The User Menu dropdown appearing in the header bar.
 */
export const UserMenu: React.FC = () => {
  return (
    <li className="nav-item dropdown" role="menu">
      <span
        className="nav-link dropdown-toggle"
        id="userMenuDropdown"
        role="button"
        data-toggle="dropdown"
        aria-haspopup="true"
        aria-expanded="false"
      >
        Graham
      </span>
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
