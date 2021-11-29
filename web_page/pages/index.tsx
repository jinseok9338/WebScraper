import type { NextPage } from "next";

const Home: NextPage = () => {
  return (
    <div className="grid grid-cols-3 grid-rows-2 gap-4 w-screen h-screen">
      <div className="h-full w-full col-span-2 bg-red-400">Hello World</div>
    </div>
  );
};

export default Home;
