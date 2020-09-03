import React from "react";
import { useTranslation } from "react-i18next";

/**
 * Spinner to indicate that data is loading
 */
export const Spinner: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="text-center">
      <div
        className="spinner-border"
        style={{ width: "3rem", height: "3rem" }}
        role="status"
      >
        <span className="sr-only">{t("components.loading")}</span>
      </div>
    </div>
  );
};
