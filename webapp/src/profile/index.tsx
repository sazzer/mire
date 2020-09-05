import React, { Suspense } from "react";
import { User, loadUser } from "../api/users";

import { Accordion } from "../components/accordion";
import { Authenticated } from "../components/authenticated";
import { Authentications } from "./authentications";
import { Breadcrumb } from "../components/breadcrumb";
import { ProfileForm } from "./profileForm";
import { Spinner } from "../components/spinner";
import { useAsyncResource } from "use-async-resource";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

interface ProfilePageProps {
  user: () => User;
}

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

interface ProfilePageLoaderProps {
  userId: string;
}

const ProfilePageLoader: React.FC<ProfilePageLoaderProps> = ({ userId }) => {
  const [user] = useAsyncResource(() => loadUser(userId, true), []);

  return <ProfilePageContents user={user} />;
};

/**
 * Page for viewing and editing the user profile
 */
export const ProfilePage: React.FC = () => {
  const { userId } = useUser();

  return (
    <Suspense fallback={<Spinner />}>
      {userId && <ProfilePageLoader userId={userId} />}
    </Suspense>
  );
};
