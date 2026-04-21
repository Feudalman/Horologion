import { Navigate, createBrowserRouter } from "react-router-dom";

import { AppShell } from "@/components/app/AppShell";
import {
  EventDetailPlaceholderPage,
  EventsPage,
} from "@/pages/events/EventsPage";
import { OverviewPage } from "@/pages/overview/OverviewPage";
import { SettingsPage } from "@/pages/settings/SettingsPage";
import {
  WindowDetailPlaceholderPage,
  WindowsPage,
} from "@/pages/windows/WindowsPage";

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
        path: "events",
        element: <EventsPage />,
      },
      {
        path: "events/:eventId",
        element: <EventDetailPlaceholderPage />,
      },
      {
        path: "windows",
        element: <WindowsPage />,
      },
      {
        path: "windows/:windowId",
        element: <WindowDetailPlaceholderPage />,
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
