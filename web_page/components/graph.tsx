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
        const res = await axios.get(url, {
          headers: {
            "Content-Type": "application/",
            "Access-Control-Allow-Origin": "*",
          },
        });
        console.log(res);
      } catch (e) {
        console.log(e);
      }
    };

    getResult("https://8000-magenta-ape-b2e7tsx7.ws-us17.gitpod.io/");
  }, []);

  return (
    <div className={`h-full w-full ${gridArea} bg-red-400`}>Hello World</div>
  );
};

export default GraphForThings;
