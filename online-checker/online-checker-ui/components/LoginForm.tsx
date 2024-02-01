"use client";

import React, { useState } from "react";

export default function LoginForm({
  setToken,
}: {
  setToken: React.Dispatch<React.SetStateAction<string | undefined>>;
}) {
  const [data, setData] = useState<LoginFormData>({
    email: "team@axum.rs",
    password: "axum.rs",
    exp_mins: 1,
  });

  const changeHandler =
    (key: keyof LoginFormData) => (e: React.ChangeEvent<HTMLInputElement>) => {
      let v = e.target.value.trim();
      if (key === "exp_mins") {
        let vi = parseInt(v, 10) || 0;
        if (!vi) {
          vi = 1;
        }
        if (vi >= 256) {
          vi = 255;
        }
        setData({ ...data, exp_mins: vi });
        return;
      }
      setData({ ...data, [key]: e.target.value.trim() });
    };

  const submitHandler = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const resp = await fetch("http://127.0.0.1:56789/login", {
      method: "POST",
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify(data),
    });
    const jsonResp: HttpJsonResponse = await resp.json();
    if (jsonResp.code !== 0) {
      window.alert(jsonResp.msg);
      return;
    }

    setToken(jsonResp.data.token);
  };

  return (
    <>
      <form
        autoComplete="off"
        className="p-6 min-w-[32rem] flex flex-col gap-y-3"
        onSubmit={submitHandler}
      >
        <div className="flex flex-col gap-y-2">
          <label
            htmlFor="email"
            className="block w-full before:text-red-600 before:content-['*'] before:mr-2"
          >
            邮箱：
          </label>
          <div className="w-full">
            <input
              id="email"
              type="email"
              className="border border-gray-300 block w-full px-3 py-1.5 focus:outline-none focus:border-gray-400"
              placeholder="输入你的邮箱"
              required
              onChange={changeHandler("email")}
              value={data["email"]}
            />
          </div>
        </div>
        <div className="flex flex-col gap-y-2">
          <label
            htmlFor="password"
            className="block w-full before:text-red-600 before:content-['*'] before:mr-2"
          >
            密码：
          </label>
          <div className="w-full">
            <input
              id="password"
              type="password"
              className="border border-gray-300 block w-full px-3 py-1.5 focus:outline-none focus:border-gray-400"
              placeholder="输入你的密码"
              required
              onChange={changeHandler("password")}
              value={data["password"]}
            />
          </div>
        </div>
        <div className="flex flex-col gap-y-2">
          <label
            htmlFor="exp_mins"
            className="block w-full before:text-red-600 before:content-['*'] before:mr-2"
          >
            过期时间（分钟）：
          </label>
          <div className="w-full">
            <input
              id="exp_mins"
              type="number"
              className="border border-gray-300 block w-full px-3 py-1.5 focus:outline-none focus:border-gray-400"
              placeholder="输入过期时间"
              required
              onChange={changeHandler("exp_mins")}
              value={data["exp_mins"]}
              min={1}
              max={255}
            />
          </div>
        </div>
        <div className="flex justify-end">
          <button className="px-3 py-1.5 bg-teal-600 text-white hover:bg-teal-700">
            登录
          </button>
        </div>
      </form>
    </>
  );
}
