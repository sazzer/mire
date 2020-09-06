import React, { Suspense } from "react";
import { resourceCache, useAsyncResource } from "use-async-resource";

import { Accordion } from "../components/accordion";
import { Authenticated } from "../components/authenticated";
import { Authentications } from "./authentications";
import { Breadcrumb } from "../components/breadcrumb";
import { ProfileForm } from "./profileForm";
import { Spinner } from "../components/spinner";
import { User } from "../api/users";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

/**
 * Props for the Profile Page
 */
interface ProfilePageProps {
  /** Reader to read the user to work with */
  user: () => User;
}

/**
 * Component to display the contents of the profile page
 * @param user The user to work with
 */
const ProfilePageContents: React.FC<ProfilePageProps> = ({ user }) => {
  const { t } = useTranslation();
  const userDetails = user();

  return (
    <>
      <Breadcrumb
        current={t("profile.breadcrumb", {
          displayName: userDetails.displayName,
        })}
        crumbs={[{ link: "/", text: t("common.home") }]}
      />

      <Authenticated>
        <Accordion
          id="userProfile"
          panes={[
            {
              title: t("profile.profile.header"),
              content: <ProfileForm user={userDetails} />,
            },
            {
              title: t("profile.authentications.header"),
              content: <Authentications user={userDetails} />,
            },
          ]}
        />
      </Authenticated>
    </>
  );
};

/**
 * Props for the profile page loader
 */
interface ProfilePageLoaderProps {
  /** Reader to read the user to work with */
  userReader: () => Promise<User>;
}

/**
 * Component to load the user from the server and display the profile page when it's loaded.
 */
const ProfilePageLoader: React.FC<ProfilePageLoaderProps> = ({
  userReader,
}) => {
  const [user] = useAsyncResource(userReader, []);

  return (
    <Suspense fallback={<Spinner />}>
      <ProfilePageContents user={user} />
    </Suspense>
  );
};

/**
 * Page for viewing and editing the user profile
 */
export const ProfilePage: React.FC = () => {
  const user = useUser();
  if (user.hasUser) {
    resourceCache(user.reload).clear();

    return <ProfilePageLoader userReader={user.reload} />;
  } else {
    return <></>;
  }
};
