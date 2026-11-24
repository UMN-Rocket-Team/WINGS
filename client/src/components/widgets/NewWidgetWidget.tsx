import { Box, Paper } from '@mui/material';
import { Add } from '@mui/icons-material';
import { WidgetContext, WidgetProvider } from './WidgetProvider';
import React from 'react';

export function NewWidgetWidget() {
    const {position, setPosition} = React.useContext(WidgetContext);
    return (
        <Paper className="h-24 items-center content-center text-center">
            <Add fontSize='large' className="text-slate-600"/>
        </Paper>
    );
}