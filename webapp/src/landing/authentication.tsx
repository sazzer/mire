import {
  Providers,
  authenticate,
  listAuthenticationProviders,
} from "../api/authentication";
import React, { Suspense } from "react";

import { useAsyncResource } from "use-async-resource";
import { useHistory } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

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
  const { setUserId } = useUser();
  const history = useHistory();

  const completeAuthentication = (userId: string) => {
    setUserId(userId);
    history.push("/characters");
  };

  return (
    <button
      className={`btn btn-block btn-social btn-${name}`}
      onClick={() => authenticate(name, completeAuthentication)}
    >
      <span className={`fa fa-${name}`}></span> {t(`authentication.${name}`)}
    </button>
  );
};

interface AuthenticationButtonsProps {
  providers: () => Providers;
}

/**
 * Render the list of authentication buttons to display
 */
const AuthenticationButtons: React.FC<AuthenticationButtonsProps> = ({
  providers,
}) => {
  return (
    <>
      {providers().map((provider) => (
        <AuthenticationButton name={provider} key={provider} />
      ))}
    </>
  );
};

/**
 * The sidebar on the landing page for logging in.
 */
export const Authentication: React.FC = () => {
  const { t } = useTranslation();
  const [providers] = useAsyncResource(() => listAuthenticationProviders(), []);

  return (
    <div>
      <h2>{t("authentication.header")}</h2>
      <Suspense fallback="Loading...">
        <AuthenticationButtons providers={providers} />
      </Suspense>
    </div>
  );
};
