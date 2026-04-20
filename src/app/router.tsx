import { Navigate, createBrowserRouter } from "react-router-dom";

import { AppShell } from "@/components/app/AppShell";
import { OverviewPage } from "@/pages/overview/OverviewPage";
import { SettingsPage } from "@/pages/settings/SettingsPage";

export const router = createBrowserRouter([
  {
    path: "/",
    element: <AppShell />,
    children: [
      {
        index: true,
        element: <Navigate replace to="/overview" />,
      },
      {
        path: "overview",
        element: <OverviewPage />,
      },
      {
        path: "settings",
        element: <SettingsPage />,
      },
      {
        path: "*",
        element: <Navigate replace to="/overview" />,
      },
    ],
  },
]);
