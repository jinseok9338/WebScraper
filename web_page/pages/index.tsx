import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";
import styles from "../styles/Home.module.css";

const Home: NextPage = () => {
  return (
    <div className="bg-red-400 w-full h-100">
      <h1 className="font-bold">Hello World</h1>
    </div>
  );
};

export default Home;
