"use client";

import React from "react";

export default function FullMask({
  children,
  className = "",
}: ChildrenProps & ClassNameProps) {
  return (
    <div className={`fixed inset-0 bg-black/30 z-[10] ${className}`}>
      <div className="bg-white mx-auto min-w-min absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 border rounded shadow">
        {children}
      </div>
    </div>
  );
}
