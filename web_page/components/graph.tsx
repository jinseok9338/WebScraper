import axios from "axios";
import { useEffect, useState } from "react";

interface GraphForThingsProps {
  gridArea: string;
}

const GraphForThings = ({ gridArea }: GraphForThingsProps) => {
  const [res, setRes] = useState([]);

  useEffect(() => {
    const getResult = async (url: string) => {
      try {
        axios.defaults.headers.get["Access-Control-Allow-Origin"] = "*";
        const res = await axios.get(url);
        console.log(res);
      } catch (e) {
        console.log(e);
      }
    };

    getResult("https://8000-plum-dragonfly-ueseoihm.ws-us17.gitpod.io/");
  }, []);

  return (
    <div className={`h-full w-full ${gridArea} bg-red-400`}>Hello World</div>
  );
};

export default GraphForThings;
