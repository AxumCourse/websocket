import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "ECHO",
  description: "AXUM中文网专题：Websocket",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="min-h-screen bg-gray-50 text-gray-700">
        <div className="container max-w-2xl  mx-auto my-6">{children}</div>
      </body>
    </html>
  );
}
