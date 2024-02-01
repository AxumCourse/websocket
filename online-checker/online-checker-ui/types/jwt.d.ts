type JwtAuthBody = {
  token: string;
};

type JwtClaims = {
  sub: string;
  iss: string;
  exp: number;
  iat: number;
  data: JwtClaimsUserData;
};

type JwtClaimsUserData = {
  id: string;
  email: string;
};
