import React from "react";
import { UserMenu } from "./usermenu";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

/**
 * Component representing the header of the entire UI.
 */
export const HeaderBar: React.FC = () => {
  const { t } = useTranslation();
  const { hasUser } = useUser();

  return (
    <header>
      <nav className="navbar navbar-expand-lg navbar-dark bg-primary">
        <span className="navbar-brand">{t("header.title")}</span>
        <button
          className="navbar-toggler"
          type="button"
          data-toggle="collapse"
          data-target="#navbarSupportedContent"
          aria-controls="navbarSupportedContent"
          aria-expanded="false"
          aria-label={t("header.toggleNavigation")}
        >
          <span className="navbar-toggler-icon"></span>
        </button>

        <div className="collapse navbar-collapse" id="navbarSupportedContent">
          <ul className="navbar-nav ml-auto">{hasUser && <UserMenu />}</ul>
        </div>
      </nav>
    </header>
  );
};
