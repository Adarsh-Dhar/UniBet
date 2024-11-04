"use client"
import { useEffect, useState } from "react";
import { useTransaction } from "../interaction/useTransaction";

const GetAllPool = () => {
  const [pools, setPools] = useState<any[]>([]);
  const { getAllPools } = useTransaction();

  useEffect(() => {
    const fetchPools = async () => {
      const result = await getAllPools();
      console.log("result", result)
      if (result) {
        setPools(result.pools || []);
      }
    };

    fetchPools();
  }, []);

  return (
    <div className="container mx-auto p-4">
      <h2 className="text-2xl font-bold mb-4">All Betting Pools</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {pools.map((pool, index) => (
          <div key={index} className="bg-gray-800 p-4 rounded-lg shadow">
            <div className="text-white">
              <p className="font-semibold">Token: {pool.token}</p>
              <p>Date: {pool.date}</p>
              <p>Amount: {pool.total_amount}</p>

              
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default GetAllPool;
