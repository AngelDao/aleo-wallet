import './App.css';
import React, {useState} from 'react';

import {Layout, Menu} from 'antd';
import {NewAccount} from "./NewAccount";
import {AccountFromPrivateKey} from "./AccountFromPrivateKey";
import DecryptRecord from "./DecryptRecord";
import { MdAccountCircle, MdAssignment } from "react-icons/md";
const {Header, Content, Footer} = Layout;

function App() {
    const [menuIndex, setMenuIndex] = useState(0);

    return (
        <Layout className="layout" style={{minHeight: '100vh'}}>
            <Header className="header">
                <div className="logo"/>
                <Menu mode="horizontal" defaultSelectedKeys={['1']}>
                    <Menu.Item key="1" onClick={() => setMenuIndex(0)}><MdAccountCircle size={20}/></Menu.Item>
                    <Menu.Item key="2" onClick={() => setMenuIndex(1)}><MdAssignment size={20}/></Menu.Item>
                    {/*<Menu.Item key="3">*/}
                    {/*    <a href="./dev/bench" target="_blank" rel="noopener noreferrer">*/}
                    {/*        Benchmarks*/}
                    {/*    </a>*/}
                    {/*</Menu.Item>*/}
                </Menu>
            </Header>
            <Content style={{padding: '100px 50px 50px 50px'}}>
                {
                    menuIndex === 0 &&
                    <>
                        <NewAccount/>
                        <br/>
                        <AccountFromPrivateKey/>
                    </>
                }
                {
                    menuIndex === 1 && <DecryptRecord/>
                }
            </Content>
            <Footer style={{textAlign: 'center'}}>Buidl by AngelDAO at ETH Denver</Footer>
        </Layout>
    );
}

export default App;
