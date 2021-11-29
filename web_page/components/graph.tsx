interface GraphForThingsProps {
  gridArea: String;
}
const GraphForThings = ({ gridArea }: GraphForThingsProps) => {
  return (
    <div className={`h-full w-full ${gridArea} bg-red-400`}>Hello World</div>
  );
};

export default GraphForThings;
