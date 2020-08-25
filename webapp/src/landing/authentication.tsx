import React from "react";
import { useTranslation } from "react-i18next";

/**
 * Props for an Authentication Button
 */
interface AuthenticationButtonProps {
  name: string;
}

/**
 * Component for a single authentication button
 * @param name The name of the button
 */
const AuthenticationButton: React.FC<AuthenticationButtonProps> = ({
  name,
}) => {
  const { t } = useTranslation();

  return (
    <button className={`btn btn-block btn-social btn-${name}`}>
      <span className={`fa fa-${name}`}></span> {t(`authentication.${name}`)}
    </button>
  );
};

/**
 * The sidebar on the landing page for logging in.
 */
export const Authentication: React.FC = () => {
  const { t } = useTranslation();

  const providers = ["facebook", "google", "twitter"];

  return (
    <div>
      <h2>{t("authentication.header")}</h2>
      {providers.map((provider) => (
        <AuthenticationButton name={provider} />
      ))}
    </div>
  );
};
