// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { describe, it } from '@jest/globals';
import 'reflect-metadata';
import 'dotenv/config';

import { Client } from '../../lib/client';
import '../customMatchers';

const client = new Client({
    nodes: [
        {
            url: process.env.NODE_URL || 'http://localhost:14265',
        },
    ],
});

// Skip for CI
describe.skip('Client info methods', () => {
    it('gets a node candidate from the synced node pool', async () => {
        const nodeInfo = await client.getNode();

        expect(nodeInfo.disabled).not.toBeTruthy();
    });

    it('gets info about node by url', async () => {
        const nodeInfo = await client.getNode();

        const nodeInfoByUrl = await client.getNodeInfo(nodeInfo.url);

        expect(nodeInfoByUrl).toBeDefined();
    });

    it('gets health of node with input url', async () => {
        const nodeInfo = await client.getNode();

        const nodeHealth = await client.getHealth(nodeInfo.url);

        expect(nodeHealth).toBeTruthy();
    });

    it('gets the unhealty nodes', async () => {
        const unhealtyNodes = await client.unhealthyNodes();

        expect(unhealtyNodes).toBeDefined();
    });

    it('gets tips', async () => {
        const tips = await client.getTips();

        expect(tips.length).toBeGreaterThan(0);
    });

    it('gets peers', async () => {
        await expect(client.getPeers()).rejects.toMatch(
            'missing or malformed jwt',
        );
    });

    it('gets networkInfo', async () => {
        const networkInfo = await client.getNetworkInfo();

        expect(networkInfo.protocolParameters.bech32Hrp).toBe('rms');
    });

    it('gets networkId', async () => {
        const networkId = await client.getNetworkId();

        expect(networkId).toBeDefined();
    });

    it('gets bech32Hrp', async () => {
        const bech32Hrp = await client.getBech32Hrp();

        expect(bech32Hrp).toBeDefined();
    });
});
