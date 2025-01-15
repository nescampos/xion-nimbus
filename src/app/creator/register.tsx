"use client";
import Link from "next/link";
import {
    Abstraxion,
    useAbstraxionAccount,
    useAbstraxionSigningClient,
  } from "@burnt-labs/abstraxion";
import { Button } from "@burnt-labs/ui";
import "@burnt-labs/ui/dist/index.css";
import type { ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { useEffect, useState } from "react";
import {nimbusContract} from "../layout";



export default function Page(): JSX.Element {
    // Abstraxion hooks
    const { data: account } = useAbstraxionAccount();
    const { client } = useAbstraxionSigningClient();

    // General state hooks
    const [isOpen, setIsOpen] = useState(false);
    const [loading, setLoading] = useState(false);
    const [creatorName, setCreatorName] = useState('');
    const [executeResult, setExecuteResult] =
        useState<ExecuteResultOrUndefined>(undefined);

    const blockExplorerUrl = `https://explorer.burnt.com/xion-testnet-1/tx/${executeResult?.transactionHash}`;

    async function registerName() {
        setLoading(true);
        const msg = {
          name:creatorName
        };
    
        try {
          const claimRes = await client?.execute(
            account.bech32Address,
            nimbusContract,
            msg,
            {
              amount: [{ amount: "0.001", denom: "uxion" }],
              gas: "500000",
            },
            "", // memo
            [],
          );
    
          setExecuteResult(claimRes);
        } catch (error) {
          // eslint-disable-next-line no-console -- No UI exists yet to display errors
          console.log(error);
        } finally {
          setLoading(false);
        }
      }

    return (
        <main className="m-auto flex min-h-screen max-w-xs flex-col items-center justify-center gap-4 p-4">
            <h1 className="text-2xl font-bold tracking-tighter text-black dark:text-white">
            Nimbus
            </h1>
            <Button
                fullWidth
                onClick={() => { setShow(true) }}
                structure="base"
            >
            {account.bech32Address ? (
                <div className="flex items-center justify-center">VIEW ACCOUNT</div>
            ) : (
                "CONNECT"
            )}
            </Button>
            {client ? (
                <label>
                    Unique name:
                    <input value={creatorName} onChange={e => setCreatorName(e.target.value)} />
                </label>
                <Button
                disabled={loading}
                fullWidth
                onClick={() => {
                    void registerName();
                }}
                structure="base"
                >
                {loading ? "LOADING..." : "Register Name"}
                </Button>
            ) : null}
            <Abstraxion
                isOpen={isOpen}
                onClose={() => {
                    setIsOpen(false);
                }}
              />
        </main>
        <label>
        First name:
        <input value={firstName} onChange={e => setFirstName(e.target.value)} />
      </label>
    )
}