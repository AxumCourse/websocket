import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "检测用户是否仍然在线",
  description: "AXUM.RS专题《Websocket》",
};

export default function RootLayout({ children }: Readonly<ChildrenProps>) {
  return (
    <html lang="zh-Hans">
      <body className="min-h-screen relative bg-gray-50 text-gray-900">
        <div className="max-w-3xl mx-auto my-6 bg-white p-6">{children}</div>
      </body>
    </html>
  );
}
