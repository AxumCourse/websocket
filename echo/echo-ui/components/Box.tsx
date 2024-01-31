import React from "react";

export default function Box({
  children,
  className = "",
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <div className={`border bg-white rounded shadow p-3 my-3 ${className}`}>
      {children}
    </div>
  );
}
