import type { NextPage } from "next";
import React from "react";
import GraphForThings from "../components/graph";

const Home: NextPage = () => {
  return (
    <div className="grid grid-cols-3 grid-rows-2 gap-4 w-screen h-screen">
      <GraphForThings gridArea="col-span-2" />
    </div>
  );
};

export default Home;
