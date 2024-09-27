import { useState, useReducer } from "react";
import Badge from '@mui/material/Badge';
import Tooltip from '@mui/material/Tooltip';
import Link from '@docusaurus/Link';
import Translate, { translate } from '@docusaurus/Translate';
import "./SimplePage.css";

import colors, { Color } from "@site/src/data/colors";
import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from '@tanstack/react-table'

const columnHelper = createColumnHelper<Color>()

export default function SimpleTable(): JSX.Element {
  const [data, _setData] = useState(() => [...colors.data])
  const rerender = useReducer(() => ({}), {})[1]
  const [locale, setLocale] = useState("zh-hans")
  const columns = [
    {
      id: "Color Name(en)",
      accessorFn: r => r.names["en"][0],
      cell: props => {
        return (
          <div style={{ width: "100%" }} onClick={(e) => {
            window.open("https://www.deepl.com/en/translator#en/" + locale + "/" + encodeURI(props.getValue()))
          }}>
            <span>{props.getValue()}</span>
          </div>
        )
      }
      ,
      footer: "Color Name(en)"
    },
    {
      id: "Color Name(" + locale + ")",
      accessorFn: (r) => {
        if (r.names && r.names[locale]) {
          return { verified: r.names[locale][0], backups: r.names[locale].slice(1) }
        }
        return { verified: "", backups: "" }
      },
      cell: props => {
        const backups = props.getValue().backups
        if (backups.length > 0) {
          return (
            <Badge badgeContent={backups.length} color="success">
              <Tooltip title={props.getValue().backups}>
                <span>{props.getValue().verified}</span>
              </Tooltip>
            </Badge>
          )
        } else {
          return <span>{props.getValue().verified}</span>
        }
      }
      ,
      footer: "Color Name(" + locale + ")"
    },
    columnHelper.accessor('hex', {
      header: () => <span>HEX</span>,
      footer: info => info.column.id,
    }),
    columnHelper.accessor('rgb', {
      header: () => 'RGB',
      footer: info => info.column.id,
    }),
    columnHelper.accessor('hsl', {
      header: () => <span>HSL</span>,
      footer: info => info.column.id,
    }),
    {
      id: "Demo",
      accessorFn: r => r.hex,
      cell: props => <div style={{ backgroundColor: props.getValue(), width: "200px" }}>&nbsp;</div>,
      footer: "Demo"
    }
  ]

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  })


  return (
    <div className="colors-table">

      <Link
        className="button button--secondary button--sm"
        to="/min">
        <Translate description="The homepage main heading">
          Click to see a minimal untranslated version
        </Translate>
      </Link>
      <table>
        <thead>
          {table.getHeaderGroups().map(headerGroup => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map(header => (
                <th key={header.id}>
                  {header.isPlaceholder
                    ? null
                    : flexRender(
                      header.column.columnDef.header,
                      header.getContext()
                    )}
                </th>
              ))}
            </tr>
          ))}
        </thead>
        <tbody>
          {table.getRowModel().rows.map(row => (
            <tr key={row.id}>
              {row.getVisibleCells().map(cell => (
                <td key={cell.id}>
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
        <tfoot>
          {table.getFooterGroups().map(footerGroup => (
            <tr key={footerGroup.id}>
              {footerGroup.headers.map(header => (
                <th key={header.id}>
                  {header.isPlaceholder
                    ? null
                    : flexRender(
                      header.column.columnDef.footer,
                      header.getContext()
                    )}
                </th>
              ))}
            </tr>
          ))}
        </tfoot>
      </table>
    </div>
  )
}