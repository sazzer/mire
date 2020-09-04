import React from "react";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

/**
 * Component that will render it's children if and only if we're currently logged in
 */
export const Authenticated: React.FC = ({ children }) => {
  const { hasUser } = useUser();
  const { t } = useTranslation();

  if (hasUser) {
    return <>{children}</>;
  } else {
    return (
      <div className="text-center">
        <svg
          width="10rem"
          height="10rem"
          viewBox="0 0 16 16"
          className="bi bi-lock"
          fill="currentColor"
          aria-hidden
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            fill-rule="evenodd"
            d="M11.5 8h-7a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1h7a1 1 0 0 0 1-1V9a1 1 0 0 0-1-1zm-7-1a2 2 0 0 0-2 2v5a2 2 0 0 0 2 2h7a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-7zm0-3a3.5 3.5 0 1 1 7 0v3h-1V4a2.5 2.5 0 0 0-5 0v3h-1V4z"
          />
        </svg>
        <br />
        {t("components.unauthenticated")}
      </div>
    );
  }
};
