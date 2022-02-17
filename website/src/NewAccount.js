import React, {useState} from "react";
import {Button, Card, Col, Divider, Form, Input, Row} from "antd";
import {CopyButton} from "./CopyButton";
import {useAleoWASM} from "./aleo-wasm-hook";

export const NewAccount = () => {
    const [account, setAccount] = useState(null);
    const [loading, setLoading] = useState(false);
    const [vanity, setVanity] = useState("");
    const aleo = useAleoWASM();

    const generateAccount = async () => {
        setLoading(true);
        setTimeout(() => {
            console.log("ALEO", aleo.Account)
            if (vanity){
                setAccount(aleo.Account.new_vanity(vanity));
                setVanity("")
            } else {
                setAccount(new aleo.Account());
            }
            setLoading(false);
        }, 1000);
    }
    const clear = () => setAccount(null);

    const privateKey = () => account !== null ? account.to_private_key() : "";
    const viewKey = () => account !== null ? account.to_view_key() : "";
    const address = () => account !== null ? account.to_address() : "";

    const layout = {labelCol: {span: 3}, wrapperCol: {span: 21}};

    if (aleo !== null) {
        return <Card title="Create a New Account" style={{width: "100%", borderRadius: "20px"}} bordered={false}>
            <Row justify="center">
                <Input size="large" placeholder="Vanity String (optional)" onChange={e => setVanity(e.target.value)} value={vanity}/>
            </Row>
            <br/>
            <Row justify="center">

                <Col><Button type="primary" shape="round" size="large" onClick={generateAccount}
                             loading={!!loading}>Generate</Button></Col>
                <Col offset="1"><Button shape="round" size="large" onClick={clear}>Clear</Button></Col>
            </Row>
            {
                account &&
                    <Form {...layout}>
                        <Divider/>
                        <Form.Item label="Private Key" colon={false}>
                            <Input size="large" placeholder="Private Key" value={privateKey()}
                                   addonAfter={<CopyButton data={privateKey()}/>} disabled/>
                        </Form.Item>
                        <Form.Item label="View Key" colon={false}>
                            <Input size="large" placeholder="View Key" value={viewKey()}
                                   addonAfter={<CopyButton data={viewKey()}/>} disabled/>
                        </Form.Item>
                        <Form.Item label="Address" colon={false}>
                            <Input size="large" placeholder="Address" value={address()}
                                   addonAfter={<CopyButton data={address()}/>} disabled/>
                        </Form.Item>
                    </Form>
            }
        </Card>
    } else {
        return <h3>
            <center>Loading...</center>
        </h3>
    }
}