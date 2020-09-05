import React, { Suspense } from "react";
import { User, loadUser } from "../api/users";
import { resourceCache, useAsyncResource } from "use-async-resource";

import { Accordion } from "../components/accordion";
import { Authenticated } from "../components/authenticated";
import { Authentications } from "./authentications";
import { Breadcrumb } from "../components/breadcrumb";
import { ProfileForm } from "./profileForm";
import { Spinner } from "../components/spinner";
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
              content: <Authentications />,
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
  /** The ID of the user */
  userId: string;
}

/**
 * Component to load the user from the server and display the profile page when it's loaded.
 */
const ProfilePageLoader: React.FC<ProfilePageLoaderProps> = ({ userId }) => {
  const [user] = useAsyncResource(loadUser, userId, true);
  return <ProfilePageContents user={user} />;
};

/**
 * Page for viewing and editing the user profile
 */
export const ProfilePage: React.FC = () => {
  const { userId } = useUser();
  resourceCache(loadUser).clear();

  return (
    <Suspense fallback={<Spinner />}>
      {userId && <ProfilePageLoader userId={userId} />}
    </Suspense>
  );
};
