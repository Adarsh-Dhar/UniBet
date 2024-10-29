const express = require("express");
const CoinpaprikaAPI = require("@coinpaprika/api-nodejs-client");
const client = new CoinpaprikaAPI();
import { Router, Request, Response } from "express";

const app = express();

const getHistoricalTickers = async (token: any) => {
  const start = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000);
  try {
    const historicalTickers = await client.getAllTickers({
      coinId: token,
      historical: {
        start: start.toISOString().slice(0, 10),
        interval: "1d",
      },
    });

    if (historicalTickers.error) throw new Error(historicalTickers.error);

    return historicalTickers.map((ticker : any) => ({
      timestamp: ticker.timestamp,
      price: ticker.price,
      marketcap: ticker.market_cap,
      volume24h: ticker.volume_24h,
    }));
    console.log(historicalTickers)
  } catch (error) {
    console.error(error);
    throw error;
  }
};

app.get("/", async (req : Request, res : Response) => {
  try {
    const token  = req.query.token;
    const data = await getHistoricalTickers(token);
    res.json(data);
  } catch (error) {
    //@ts-ignore
    res.status(500).json({ error: error.message });
  }
});

app.listen(5000, () => {
  console.log("Server running on http://localhost:5000");
});
