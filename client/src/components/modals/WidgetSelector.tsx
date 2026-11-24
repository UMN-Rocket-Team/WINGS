//placeholder widget selector
//dynamically renders selected widget based on user selection


import Dialog from '@mui/material/Dialog';
import Button from '@mui/material/Button';
import * as React from 'react';
import { DialogContent, Typography } from '@mui/material';

function Widget1() {
  return <div>Widget 1 placeholder</div>;
}

function Widget2() {
  return <div>Widget 2 placeholder</div>;
}

function Widget3() {
  return <div>Widget 3 placeholder</div>;
}

enum Widgets {
    Widget1 = "widget1",
    Widget2 = "widget2",
    Widget3 = "widget3"
}

const WidgetElements = {
    widget1: <Widget1 />,
    widget2: <Widget2 />,
    widget3: <Widget3 />
}



export function WidgetSelector() {
  const [isOpen, setIsOpen] = React.useState(false);
  const [selectedWidget, setSelectedWidget] = React.useState<Widgets | null>(null);

  return (
    <>
      <Button variant="contained" onClick={() => setIsOpen(true)}>
        Select Widget
      </Button>

      <Dialog
        open={isOpen}
        onClose={() => setIsOpen(false)}
      >
        <DialogContent>
          <Typography>{selectedWidget && WidgetElements[selectedWidget]}</Typography>
        </DialogContent>
      </Dialog>
    </>
  );
}