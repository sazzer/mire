import { Authenticated } from "../components/authenticated";
import { Breadcrumb } from "../components/breadcrumb";
import React from "react";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

/**
 * Page for viewing and editing the user profile
 */
export const ProfilePage: React.FC = () => {
  const { t } = useTranslation();
  const { user } = useUser();

  return (
    <>
      <Breadcrumb
        current={t("profile.breadcrumb", { displayName: user?.displayName })}
        crumbs={[{ link: "/", text: t("common.home") }]}
      />

      <Authenticated>
        <div>Hello</div>
      </Authenticated>
    </>
  );
};
